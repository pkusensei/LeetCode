using System.Text;
using Solution.LList;
// using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public Node CopyRandomList(Node head)
    {
        if (head is null) { return null; }
        var curr = head;
        while (curr is not null)
        {
            Node new_node = new(curr.val)
            {
                next = curr.next,
                random = curr.random,
            };
            curr.next = new_node;
            curr = new_node.next;
        }
        curr = head.next;
        while (curr is not null)
        {
            if (curr.random is not null)
            {
                curr.random = curr.random.next;
            }
            curr = curr.next?.next;
        }
        var old = head;
        var res = head.next;
        curr = res;
        while (curr is not null)
        {
            old.next = curr.next;
            old = old.next;
            curr.next = curr.next?.next;
            curr = curr.next;
        }
        return res;
    }
}