using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public long MinTime(int[] skill, int[] mana)
    {
        int n = skill.Length;
        long[] finish = new long[1 + n];
        foreach (var m in mana)
        {
            for (int i = 0; i < n; i++)
            {
                // Max of [1+i] on previous potion
                //     vs [i] on current potion
                finish[1 + i] = long.Max(finish[1 + i], finish[i]) + m * skill[i];
            }
            for (int i = n - 1; i >= 0; i -= 1)
            {
                finish[i] = finish[1 + i] - m * skill[i];
            }
        }
        return finish[^1];
    }
}