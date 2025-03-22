using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<IList<int>> ClosestNodes(TreeNode root, IList<int> queries)
    {
        List<int> nums = [];
        Flatten(root);
        List<IList<int>> res = [];
        foreach (var q in queries)
        {
            int i = nums.BinarySearch(q);
            if (i >= 0) { res.Add([q, q]); }
            else
            {
                i = ~i;
                if (i == 0) { res.Add([-1, nums[i]]); }
                else if (i == nums.Count) { res.Add([nums[i - 1], -1]); }
                else { res.Add([nums[i - 1], nums[i]]); }
            }
        }
        return res;

        void Flatten(TreeNode node)
        {
            if (node is null) { return; }
            Flatten(node.left);
            nums.Add(node.val);
            Flatten(node.right);
        }
    }
}
