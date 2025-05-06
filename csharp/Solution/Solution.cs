using System.Buffers;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int[] BuildArray(int[] nums)
    {
        return nums.Select(v => nums[v]).ToArray();
    }
}
