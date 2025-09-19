using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumDecodings(string s)
    {
        const long M = 1_000_000_007;
        long prev = 1;
        long curr = s[0] switch
        {
            '0' => 0,
            '*' => 9,
            _ => 1,
        };
        for (int i = 0; i < s.Length - 1; i++)
        {
            long temp = curr;
            if (s[1 + i] == '*')
            {
                curr = curr * 9 % M;
                switch (s[i])
                {
                    case '1':
                        curr = (curr + 9 * prev) % M;
                        break;
                    case '2':
                        curr = (curr + 6 * prev) % M;
                        break;
                    case '*':
                        curr = (curr + 15 * prev) % M;
                        break;
                    default:
                        break;
                }
            }
            else
            {
                if (s[1 + i] == '0') { curr = 0; }
                switch (s[i])
                {
                    case '1':
                        curr = (curr + prev) % M;
                        break;
                    case '2' when s[1 + i] is <= '6':
                        curr = (curr + prev) % M;
                        break;
                    case '*' when s[1 + i] is <= '6':
                        curr = (curr + 2 * prev) % M;
                        break;
                    case '*':
                        curr = (curr + prev) % M;
                        break;
                    default:
                        break;
                }
            }
            prev = temp;
        }
        return (int)curr;
    }
}
