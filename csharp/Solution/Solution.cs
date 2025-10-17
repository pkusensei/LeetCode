using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Solution(int n, int[] blacklist)
    {
        Rng = new();
        N = n - blacklist.Length;
        Banned = [];
        HashSet<int> set = [.. blacklist];
        int val = N;
        foreach (var banned in blacklist.Order())
        {
            if (banned >= N) { break; }
            while (set.Contains(val)) { val += 1; }
            Banned.Add(banned, val);
            val += 1;
        }
    }

    Random Rng { get; }
    int N { get; }
    Dictionary<int, int> Banned { get; }

    public int Pick()
    {
        int v = Rng.Next(N);
        return Banned.GetValueOrDefault(v, v);
    }
}

