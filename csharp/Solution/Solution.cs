using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(int[] w)
    {
        Prefix = new int[w.Length];
        Rng = new();
        int prev = 0;
        for (int i = 0; i < w.Length; i++)
        {
            Prefix[i] = prev + w[i];
            prev = Prefix[i];
        }
    }

    int[] Prefix { get; }
    Random Rng { get; }

    public int PickIndex()
    {
        int val = Rng.Next(1, 1 + Prefix[^1]);
        int i = Array.BinarySearch(Prefix, val);
        return i < 0 ? ~i : i;
    }
}
