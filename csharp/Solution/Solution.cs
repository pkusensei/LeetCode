using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool DivideArray(int[] nums)
    => nums.GroupBy(x => x).Select(g => g.Count()).All(v => (v & 1) == 0);
}
