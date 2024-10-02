// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public IList<int> Postorder(Node root)
    {
        List<int> res = [];
        if (root is null) { return res; }
        var st = new Stack<Node>();
        st.Push(root);
        while (st.TryPop(out var node))
        {
            if (node is not null)
            {
                res.Add(node.val);
                foreach (var item in node.children)
                {
                    st.Push(item);
                }
            }
        }
        res.Reverse();
        return res;
    }
}
