// See https://aka.ms/new-console-template for more information

using System.Text;

Test1();
Test2();
Test3();


void Test1()
{
    ListNode n = new(1, new(2, new(3, new(4))));
    var ans = SwapPairs(n);
    Console.WriteLine(ans); // [2,1,4,3]
}

void Test2()
{
    Console.WriteLine(SwapPairs(null));
}

void Test3()
{
    Console.WriteLine(SwapPairs(new(1))); // []
}


ListNode SwapPairs(ListNode head)
{
    ListNode lst = new(0, head);
    (ListNode node, int step) fast = (lst, 0);
    (ListNode node, int step) slow = (lst, 0);

    while (fast.node is not null)
    {
        fast.node = fast.node.next;
        fast.step += 1;

        if (slow.step + 2 < fast.step)
        {
            slow.node = slow.node.next;
            slow.step += 1;
        }

        if ((fast.step & 1) == 0 && fast.node is not null)
        {
            // slow prev fast 
            //  0    1    2    3
            //  0    2    1    3
            var prev = slow.node.next;
            prev.next = fast.node.next;
            fast.node.next = prev;
            slow.node.next = fast.node;
            fast.node = slow.node.next.next;
        }
    }

    return lst.next;
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
