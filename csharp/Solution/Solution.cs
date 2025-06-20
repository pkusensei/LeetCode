using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxDistance(string s, int k)
    {
        int res = 0;
        int leftright = 0;
        int updown = 0;
        for (int i = 0; i < s.Length; i++)
        {
            switch (s[i])
            {
                case 'E': leftright += 1; break;
                case 'W': leftright -= 1; break;
                case 'N': updown += 1; break;
                case 'S': updown -= 1; break;
                default: break;
            }
            int curr = Math.Min(1 + i, Math.Abs(leftright) + Math.Abs(updown) + 2 * k);
            res = Math.Max(res, curr);
        }
        return res;
    }
}
