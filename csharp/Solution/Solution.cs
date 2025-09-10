using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinimumTeachings(int n, int[][] languages, int[][] friendships)
    {
        HashSet<int> needs = [];
        foreach (var f in friendships)
        {
            if (!languages[f[0] - 1].Intersect(languages[f[1] - 1]).Any())
            {
                needs.UnionWith([f[0] - 1, f[1] - 1]); // Both needs a language
            }
        }
        int[] freq = new int[1 + n];
        int max = 0;
        foreach (var f in needs)
        {
            foreach (var item in languages[f])
            {
                freq[item] += 1;
                max = int.Max(max, freq[item]);
            }
        }
        return needs.Count - max;
    }
}
