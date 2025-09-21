using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class MovieRentingSystem
{
    public MovieRentingSystem(int n, int[][] entries)
    {
        InStore = [];
        Rented = [];
        Dictionary<(int, int), int> lookup = [];
        foreach (var item in entries)
        {
            int shop = item[0];
            int movie = item[1];
            int price = item[2];
            InStore.TryAdd(movie, []);
            InStore[movie].Add((price, shop));
            lookup.Add((shop, movie), price);
        }
        Lookup = lookup.ToFrozenDictionary();
    }

    // movie_id - (price, shop)
    Dictionary<int, SortedSet<(int price, int shop)>> InStore = [];
    SortedSet<Movie> Rented { get; }
    FrozenDictionary<(int shop, int movie), int> Lookup { get; }

    public IList<int> Search(int movie)
    {
        List<int> res = [];
        if (!InStore.TryGetValue(movie, out var set)) { return res; }
        foreach (var (_, shop) in set)
        {
            res.Add(shop);
            if (res.Count == 5) { break; }
        }
        return res;
    }

    public void Rent(int shop, int movie)
    {
        int price = Lookup[(shop, movie)];
        InStore[movie].Remove((price, shop));
        Rented.Add(new(shop, movie, price));
    }

    public void Drop(int shop, int movie)
    {
        int price = Lookup[(shop, movie)];
        InStore[movie].Add((price, shop));
        Rented.Remove(new(shop, movie, price));
    }

    public IList<IList<int>> Report() => [.. Rented.Take(5).Select(m => (IList<int>)[m.Shop, m.Id])];

    readonly record struct Movie(int Shop, int Id, int Price) : IComparable<Movie>
    {
        public int CompareTo(Movie other)
        {
            if (Price != other.Price) { return Price.CompareTo(other.Price); }
            else if (Shop != other.Shop) { return Shop.CompareTo(other.Shop); }
            else { return Id.CompareTo(other.Id); }
        }
    }
}