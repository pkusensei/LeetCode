using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[][] ReconstructQueue(int[][] people)
    {
        int n = people.Length;
        Array.Sort(people, (a, b) =>
        {
            if (a[0] == b[0]) { return a[1].CompareTo(b[1]); }
            return b[0].CompareTo(a[0]);
        });
        List<int[]> res = new(n);
        foreach (var p in people)
        {
            if (p[1] >= res.Count) { res.Add(p); }
            else { res.Insert(p[1], p); }
        }
        return [.. res];
    }
}
