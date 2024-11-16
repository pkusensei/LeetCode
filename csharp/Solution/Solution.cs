using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public string SmallestFromLeaf(TreeNode root)
    {
        var res = "";
        Backtrack(root, [], ref res);
        return res;
    }

    static void Backtrack(TreeNode node, List<char> curr, ref string res)
    {
        if (node is null) { return; }
        curr.Insert(0, (char)('a' + node.val));
        if (node.left is null && node.right is null)
        {
            var s = string.Join(null, curr);
            if (res.Length == 0 || s.CompareTo(res) < 0)
            {
                res = s;
            }
        }
        else
        {
            Backtrack(node.left, curr, ref res);
            Backtrack(node.right, curr, ref res);
        }
        curr.RemoveAt(0);
    }
}
