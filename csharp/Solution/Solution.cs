using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public TreeNode RecoverFromPreorder(string traversal)
    {
        Stack<(int level, TreeNode node)> st = [];
        int level = 0;
        int num = 0;
        (int level, TreeNode node) top;
        TreeNode node;
        foreach (var c in traversal)
        {
            if (c == '-')
            {
                if (num > 0)
                {
                    node = new(num);
                    while (st.TryPeek(out top) && top.level >= level)
                    {
                        st.Pop();
                    }
                    if (st.TryPeek(out top))
                    {
                        if (top.node.left is null) { top.node.left = node; }
                        else { top.node.right = node; }
                    }
                    st.Push((level, node));
                    num = 0;
                    level = 0;
                }
                level += 1;
            }
            else
            {
                num = 10 * num + c - '0';
            }
        }
        node = new(num);
        while (st.TryPeek(out top) && top.level >= level)
        {
            st.Pop();
        }
        if (st.TryPeek(out top))
        {
            if (top.node.left is null) { top.node.left = node; }
            else { top.node.right = node; }
        }
        st.Push((level, node));
        return st.Last().node;
    }
}

