using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinFlipsMonoIncr(string s)
    {
        int n = s.Length;
        List<int> pref1 = new(n);
        foreach (var item in s)
        {
            pref1.Add((item == '1' ? 1 : 0) + pref1.LastOrDefault());
        }
        List<int> suf0 = new(n);
        foreach (var item in s.Reverse())
        {
            suf0.Add((item == '0' ? 1 : 0) + suf0.LastOrDefault());
        }
        suf0.Reverse();
        return pref1.Zip(suf0).Select(p => p.First + p.Second - 1).Min();
    }
}