using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(int m, int n)
    {
        M = m;
        N = n;
        Rng = new();
        Seen = [];
    }

    public int M { get; }
    public int N { get; }
    Random Rng { get; }
    HashSet<int> Seen { get; }

    public int[] Flip()
    {
        int val = Rng.Next(M * N - Seen.Count);
        for (; !Seen.Add(val); val += 1) { }
        int y = val / M;
        int x = val % M;
        return [x, y];
    }

    public void Reset() => Seen.Clear();
}