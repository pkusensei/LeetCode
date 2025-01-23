using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int MaxUniqueSplit(string s)
    {
        HashSet<string> set = [];
        var res = 0;
        Backtrack(s);
        return res;

        void Backtrack(string s)
        {
            if (set.Count + s.Length <= res) { return; }
            if (s.Length == 0) { res = Math.Max(res, set.Count); }
            for (int i = 1; i <= s.Length; i++)
            {
                if (set.Add(s[..i]))
                {
                    Backtrack(s[i..]);
                    set.Remove(s[..i]);
                }
            }
        }
    }
}
