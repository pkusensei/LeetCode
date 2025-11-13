using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int NumRabbits(int[] answers)
    {
        int res = 0;
        foreach (var (key, val) in answers.CountBy(x => x))
        {
            int pack = 1 + key;
            res += (int)Math.Ceiling((double)val / pack) * pack;
        }
        return res;
    }
}

