using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Node Flatten(Node head)
    {
        if (head is null) { return null; }
        return Dfs(head).n1;

        static (Node n1, Node n2) Dfs(Node head)
        {
            var curr = head;
            while (curr.next is not null)
            {
                if (curr.child is not null)
                {
                    var temp = curr.child;
                    curr.child = null;
                    var next = curr.next;
                    (var n1, var n2) = Dfs(temp);
                    curr.next = n1;
                    n1.prev = curr;
                    n2.next = next;
                    next.prev = n2;
                }
                curr = curr.next;
            }
            if (curr.child is not null)
            {
                (var n1, var n2) = Dfs(curr.child);
                curr.child = null;
                curr.next = n1;
                n1.prev = curr;
                curr = n2;
            }
            return (head, curr);
        }
    }
}

public class Node
{
    public int val;
    public Node prev;
    public Node next;
    public Node child;
}