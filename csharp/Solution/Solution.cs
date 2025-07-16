using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<IList<string>> Partition(string s)
    {
        List<IList<string>> res = [];
        Backtrack(s, []);
        return res;

        void Backtrack(ReadOnlySpan<char> s, List<string> curr)
        {
            if (s.IsEmpty)
            {
                res.Add([.. curr]);
                return;
            }
            for (int i = 0; i < s.Length; i++)
            {
                if (IsPalindrome(s[..(1 + i)]))
                {
                    curr.Add(new(s[..(1 + i)]));
                    Backtrack(s[(1 + i)..], curr);
                    curr.RemoveAt(curr.Count - 1);
                }
            }
        }

        static bool IsPalindrome(ReadOnlySpan<char> s)
        {
            if (s.Length <= 1) { return true; }
            for (int i = 0; i < s.Length; i++)
            {
                if (s[i] != s[s.Length - 1 - i]) { return false; }
            }
            return true;
        }
    }
}