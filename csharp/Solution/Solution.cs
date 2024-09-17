// using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public Node Flatten(Node head)
    {
        if (head is null) { return null; }
        Node dummy = new() { val = 0, next = head };
        Solve(dummy.next);
        return dummy.next;
    }

    static Node Solve(Node node)
    {
        var curr = node;
        var prev = curr.prev;
        while (curr is not null)
        {
            if (curr.child is not null)
            {
                var temp = curr.next;
                curr.child.prev = curr;
                curr.next = curr.child;
                curr.child = null;
                var tail = Solve(curr.next);
                if (temp is not null)
                {
                    tail.next = temp;
                    temp.prev = tail;
                }
            }
            prev = curr;
            curr = curr.next;
        }
        return prev;
    }
}

public class Node
{
    public int val;
    public Node prev;
    public Node next;
    public Node child;
}