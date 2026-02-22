using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode ConstructFromPrePost(int[] preorder, int[] postorder)
    {
        int n = preorder.Length;
        Dictionary<int, int> predict = [];
        Dictionary<int, int> postdict = [];
        for (int i = 0; i < n; i++)
        {
            predict.Add(preorder[i], i);
            postdict.Add(postorder[i], i);
        }
        return Dfs(0, n - 1, 0, n - 1);

        TreeNode Dfs(int pre_left, int pre_right, int post_left, int post_right)
        {
            if (pre_left > pre_right) { return null; }
            TreeNode root = new(preorder[pre_left]);
            if (pre_left == pre_right) { return root; }
            int left_val = preorder[pre_left + 1];
            int right_val = postorder[post_right - 1];
            int left_start = 1 + pre_left;
            int left_end = predict[right_val] - 1;
            int right_start = post_left;
            int right_end = postdict[left_val];
            root.left = Dfs(left_start, left_end, right_start, right_end);

            left_start = predict[right_val];
            left_end = pre_right;
            right_start = 1 + postdict[left_val];
            right_end = post_right - 1;
            root.right = Dfs(left_start, left_end, right_start, right_end);
            return root;
        }
    }
}