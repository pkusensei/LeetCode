using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class FoodRatings
{
    public FoodRatings(string[] foods, string[] cuisines, int[] ratings)
    {
        F_CR = [];
        C_RF = [];
        foreach (var (food, (cui, rat)) in foods.Zip(cuisines.Zip(ratings)))
        {
            F_CR.Add(food, (cui, rat));
            C_RF.TryAdd(cui,
                new(Comparer<(int rat, string food)>.Create(
                    (a, b) => a.rat == b.rat ? b.food.CompareTo(a.food) : a.rat.CompareTo(b.rat))));
            C_RF[cui].Add((rat, food));
        }
    }

    Dictionary<string, (string cui, int rat)> F_CR { get; }
    Dictionary<string, SortedSet<(int rat, string food)>> C_RF { get; }

    public void ChangeRating(string food, int newRating)
    {
        (var cui, var curr_rat) = F_CR[food];
        F_CR[food] = (cui, newRating);
        C_RF[cui].Remove((curr_rat, food));
        C_RF[cui].Add((newRating, food));
    }

    public string HighestRated(string cuisine) => C_RF[cuisine].Max.food;
}