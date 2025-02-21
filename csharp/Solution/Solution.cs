using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
}

public class FindElements
{
    HashSet<int> Nums { get; }
    public FindElements(TreeNode root)
    {
        root.val = 0;
        Nums = [];
        Dfs(root);

        void Dfs(TreeNode node)
        {
            Nums.Add(node.val);
            if (node.left is not null)
            {
                node.left.val = 1 + 2 * node.val;
                Dfs(node.left);
            }
            if (node.right is not null)
            {
                node.right.val = 2 + 2 * node.val;
                Dfs(node.right);
            }
        }
    }

    public bool Find(int target) => Nums.Contains(target);
}
