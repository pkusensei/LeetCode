using System.Text;

Test1();
Test2();


void Test1()
{
    ListNode n = new(1, new(2, new(3, new(4, new(5)))));
    var ans = ReverseKGroup(n, 2);
    Console.WriteLine(ans); // [2,1,4,3,5]
}

void Test2()
{
    ListNode n = new(1, new(2, new(3, new(4, new(5)))));
    var ans = ReverseKGroup(n, 3);
    Console.WriteLine(ans); // [3,2,1,4,5]
}


ListNode ReverseKGroup(ListNode head, int k)
{
    if (k < 2) { return head; }

    ListNode lst = new(0, head);
    (ListNode node, int step) fast = (lst, 0);
    (ListNode node, int step) slow = (lst, 0);

    while (fast.node is not null)
    {
        fast.node = fast.node.next;
        fast.step += 1;

        if (slow.step + k < fast.step)
        {
            slow.node = slow.node.next;
            slow.step += 1;
        }

        if (fast.step % k == 0 && fast.node is not null)
        {
            // slow first     fast next
            //  0    1    2    3    4
            //  0    3    2    1    4
            var next = fast.node.next;
            var first = slow.node.next;
            var (left, right) = ReverseK(first, fast.node, k);
            slow.node.next = left;
            right.next = next;
            fast.node = right;
        }
    }
    return lst.next;
}

(ListNode, ListNode) ReverseK(ListNode left, ListNode right, int k)
{
    if (k == 2) { return SwapPair(left); }

    var first = left;
    left = left.next;
    (left, right) = ReverseK(left, right, k - 1);
    first.next = null;
    right.next = first;
    return (left, first);
}

(ListNode, ListNode) SwapPair(ListNode head)
{
    var left = head.next;
    var right = head;
    left.next = right;
    right.next = null;
    return (left, right);
}

class ListNode
{
    public int val;
    public ListNode next;
    public ListNode(int val = 0, ListNode next = null)
    {
        this.val = val;
        this.next = next;
    }

    public override string ToString()
    {
        var curr = this;
        var sb = new StringBuilder("[");
        while (curr is ListNode n)
        {
            sb.AppendFormat("{0}, ", n.val);
            curr = n.next;
        }
        sb.Remove(sb.Length - 2, 2);
        sb.Append(']');
        return sb.ToString();
    }
}
