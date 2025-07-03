using System.Security.Principal;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public char KthCharacter(int k)
    {
        int len = (int)Math.Pow(2, Math.Ceiling(Math.Log2(k)));
        return Dfs('a', k, len);

        static char Dfs(char curr, int k, int len)
        {
            if (k == 1) { return curr; }
            int half = len / 2;
            if (k <= half) { return Dfs(curr, k, half); }
            else
            {
                char next = curr == 'z' ? 'a' : (char)(curr + 1);
                return Dfs(next, k - half, half);
            }
        }
    }
}
