using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class FindElements
{
    public HashSet<int> Nums { get; }

    public FindElements(TreeNode root)
    {
        Nums = [];
        Dfs(root, 0);

        void Dfs(TreeNode node, int val)
        {
            if (node is null) { return; }
            node.val = val;
            Nums.Add(val);
            Dfs(node.left, 1 + 2 * val);
            Dfs(node.right, 2 + 2 * val);
        }
    }

    public bool Find(int target) => Nums.Contains(target);
}