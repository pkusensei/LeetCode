// See https://aka.ms/new-console-template for more information

using System.Text;

Test1();
Test2();
Test3();


void Test1()
{
    ListNode n1 = new(1, new(4, new(5)));
    ListNode n2 = new(1, new(3, new(4)));
    ListNode n3 = new(2, new(6));
    var ans = MergeKLists([n1, n2, n3]);
    Console.WriteLine(ans); // [1,1,2,3,4,4,5,6]
}

void Test2()
{
    Console.WriteLine(MergeKLists([]));
}

void Test3()
{
    Console.WriteLine(MergeKLists([null])); // []
}


ListNode MergeKLists(ListNode[] lists)
{
    var queue = new PriorityQueue<ListNode, int>();
    foreach (var item in lists)
    {
        if (item is ListNode node)
        {
            queue.Enqueue(node, node.val);
        }
    }
    if (!queue.TryDequeue(out var head, out var _))
    {
        return null;
    }

    var curr = head;
    if (head is ListNode n && n.next is not null)
    {
        queue.Enqueue(n.next, n.next.val);
    }

    while (queue.TryDequeue(out var node, out var _))
    {
        curr.next = node;
        curr = curr.next;
        node = node.next;
        if (node is not null)
        {
            queue.Enqueue(node, node.val);
        }
    }

    return head;
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
