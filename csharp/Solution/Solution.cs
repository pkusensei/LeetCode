using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string DecodeString(string s)
    {
        return Dfs(s);

        static string Dfs(ReadOnlySpan<char> s)
        {
            StringBuilder sb = new();
            int open = 0;
            int num = 0;
            for (int i = 0; i < s.Length; i++)
            {
                if (char.IsAsciiLetter(s[i])) { sb.Append(s[i]); }
                else if (char.IsAsciiDigit(s[i])) { num = 10 * num + s[i] - '0'; }
                else if (s[i] == '[')
                {
                    int left = 1 + i;
                    open += 1;
                    for (i = left; i < s.Length && open > 0; i += 1)
                    {
                        if (s[i] == '[') { open += 1; }
                        if (s[i] == ']') { open -= 1; }
                    }
                    i -= 1;
                    var inner = Dfs(s[left..i]);
                    for (int _ = 0; _ < num; _++)
                    {
                        sb.Append(inner);
                    }
                    num = 0;
                }
            }
            return sb.ToString();
        }
    }
}
