using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MajorityElement(int[] nums)
    {
        int res = 0;
        int count = 0;
        foreach (var num in nums)
        {
            if (num == res) { count += 1; }
            else
            {
                count -= 1;
                if (count <= 0)
                {
                    res = num;
                    count = 1;
                }
            }
        }
        return res;
    }
}
