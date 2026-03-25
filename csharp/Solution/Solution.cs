using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int BagOfTokensScore(int[] tokens, int power)
    {
        int n = tokens.Length;
        if (n == 0) { return 0; }
        Array.Sort(tokens);
        int left = 0;
        int right = n - 1;
        int score = 0;
        int res = 0;
        while (left <= right)
        {
            bool moved = false;
            while (left <= right && power >= tokens[left])
            {
                power -= tokens[left];
                score += 1;
                left += 1;
                moved = true;
            }
            res = int.Max(res, score);
            if (left <= right && score > 0)
            {
                score -= 1;
                power += tokens[right];
                right -= 1;
                moved = true;
            }
            if (!moved) { break; }
        }
        return res;
    }
}
