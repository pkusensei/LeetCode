using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string MakeLargestSpecial(string s)
    {
        return Dfs(s);

        static string Dfs(ReadOnlySpan<char> s)
        {
            if (s.Length <= 2) { return new(s); }
            int open = 0;
            int left = 0;
            List<string> res = [];
            StringBuilder sb = new();
            for (int right = 0; right < s.Length; right++)
            {
                if (s[right] == '1') { open += 1; }
                else { open -= 1; }
                if (open == 0)
                {
                    sb.Clear();
                    sb.Append('1');
                    sb.Append(Dfs(s[(1 + left)..right]));
                    sb.Append('0');
                    res.Add(sb.ToString());
                    left = 1 + right;
                }
            }
            res.Sort((a, b) => b.CompareTo(a));
            return string.Join("", res);
        }
    }
}