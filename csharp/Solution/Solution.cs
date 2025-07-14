using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> RestoreIpAddresses(string s)
    {
        List<string> res = [];
        Backtrack(s, []);
        return res;

        void Backtrack(ReadOnlySpan<char> s, List<int> curr)
        {
            if (s.IsEmpty && curr.Count == 4)
            {
                res.Add(string.Join('.', curr));
                return;
            }
            if (s.IsEmpty || curr.Count >= 4) { return; }
            for (int len = 1; len <= int.Min(3, s.Length); len++)
            {
                int val = int.Parse(s[..len]);
                if (len == 1 || (len > 1 && s[0] > '0' && val < 256))
                {
                    curr.Add(val);
                    Backtrack(s[len..], curr);
                    curr.RemoveAt(curr.Count - 1);
                }
            }
        }
    }
}