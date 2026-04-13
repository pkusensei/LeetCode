using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public IList<int> FlipMatchVoyage(TreeNode root, int[] voyage)
    {
        List<int> res = [];
        return Dfs(root, voyage) ? res : [-1];

        bool Dfs(TreeNode node, ReadOnlySpan<int> arr)
        {
            if (node is null) { return arr.IsEmpty; }
            if (arr.IsEmpty || node.val != arr[0]) { return false; }
            if (node.left is null) { return Dfs(node.right, arr[1..]); }
            if (node.right is null) { return Dfs(node.left, arr[1..]); }
            if (arr.Length < 2) { return false; }
            if (arr[1] == node.left.val)
            {
                int right = arr.IndexOf(node.right.val);
                return right >= 2 && Dfs(node.left, arr[1..right]) && Dfs(node.right, arr[right..]);
            }
            else
            {
                res.Add(node.val);
                int left = arr.IndexOf(node.left.val);
                return left >= 2 && Dfs(node.right, arr[1..left]) && Dfs(node.left, arr[left..]);
            }
        }
    }
}
