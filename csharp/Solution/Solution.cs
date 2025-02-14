using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class ProductOfNumbers
{
    List<int> Nums { get; }
    public ProductOfNumbers() => Nums = [];

    public void Add(int num)
    {
        if (num == 0) { Nums.Clear(); }
        else if (Nums.LastOrDefault() == 0) { Nums.Add(num); }
        else { Nums.Add(num * Nums.Last()); }
    }

    public int GetProduct(int k)
    {
        var i = Nums.Count - k;
        if (i < 0) { return 0; }
        if (i == 0) { return Nums.Last(); }
        return Nums.Last() / Nums[i - 1];
    }
}
