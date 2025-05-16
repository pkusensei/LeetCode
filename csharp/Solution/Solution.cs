using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<string> GetWordsInLongestSubsequence(string[] words, int[] groups)
    {
        int n = words.Length;
        Span<int> dp = stackalloc int[n];
        dp.Fill(1);
        Span<int> prev = stackalloc int[n];
        prev.Fill(-1);
        (int max, int maxi) = (0, 0);
        for (int right = 1; right < n; right++)
        {
            for (int left = 0; left < right; left++)
            {
                if (Check(left, right) && dp[left] + 1 > dp[right])
                {
                    dp[right] = 1 + dp[left];
                    prev[right] = left;
                    if (dp[right] > max)
                    {
                        max = dp[right];
                        maxi = right;
                    }
                }
            }
        }
        List<string> res = new(max);
        while (maxi >= 0)
        {
            res.Add(words[maxi]);
            maxi = prev[maxi];
        }
        res.Reverse();
        return res;

        bool Check(int left, int right)
        {
            var a = words[left];
            var b = words[right];
            return groups[left] != groups[right]
                && a.Length == b.Length
                && a.Zip(b).Count(p => p.First != p.Second) == 1;
        }
    }
}
