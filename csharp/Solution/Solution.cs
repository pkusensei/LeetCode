using System.Collections.Frozen;
using System.Linq.Expressions;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] SmallestSufficientTeam(string[] req_skills, IList<IList<string>> people)
    {
        int n = req_skills.Length;
        Dictionary<string, int> str_i = [];
        for (int i = 0; i < n; i++)
        {
            str_i[req_skills[i]] = i;
        }
        int[] people_mask = people.Select(list =>
        {
            int mask = 0;
            foreach (var item in list)
            {
                mask |= 1 << str_i[item];
            }
            return mask;
        }).ToArray();
        List<int>[] dp = new List<int>[1 << n];
        dp[0] = [];
        for (int mask = 0; mask < (1 << n) - 1; mask++)
        {
            if (dp[mask] is null) { continue; }
            for (int i = 0; i < people_mask.Length; i++)
            {
                int nmask = mask | people_mask[i];
                if (int.PopCount(nmask) > int.PopCount(mask) &&
                (dp[nmask] is null || dp[nmask].Count > 1 + dp[mask].Count))
                {
                    dp[nmask] = [.. dp[mask]];
                    dp[nmask].Add(i);
                }
            }
        }
        return [.. dp[^1]];
    }
}
