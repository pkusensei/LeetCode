using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MagicalString(int n)
    {
        List<int> s = new(1 + n) { 1, 2, 2 };
        int i = 2;
        while (s.Count < n)
        {
            int item = 3 - s.Last();
            s.AddRange(Enumerable.Range(0, s[i]).Select(_ => item));
            i += 1;
        }
        return s.Take(n).Count(v => v == 1);
    }
}