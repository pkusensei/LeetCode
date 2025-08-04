using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(int[] nums)
    {
        Rng = new();
        Nums = nums;
    }

    Random Rng { get; }
    int[] Nums { get; }

    public int[] Reset() => Nums;

    public int[] Shuffle()
    {
        int[] res = [.. Nums];
        Rng.Shuffle(res);
        return res;
    }
}