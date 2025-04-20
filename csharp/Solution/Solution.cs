using System.Numerics;
using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int NumRabbits(int[] answers)
    {
        int res = 0;
        foreach (var (key, val) in answers.CountBy(x => x))
        {
            int group = 1 + key;
            res += (int)Math.Ceiling(1.0 * val / group) * group; // cannot see self
        }
        return res;
    }
}
