using System.Collections;
using System.Diagnostics;
using System.Text;

Test1();
Test2();
Test3();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    ListNode n = Partition(ListNode.Make([1, 4, 3, 2, 5, 2]), 3);
    var a = "[1,2,2,4,3,5]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    ListNode n = Partition(ListNode.Make([2, 1]), 2);
    var a = "[1,2]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test3()
{
    ListNode n = Partition(ListNode.Make([4, 3, 2, 5, 2]), 3);
    var a = "[2,2,4,3,5]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

ListNode Partition(ListNode head, int x)
{
    if (head is null) { return head; }

    (ListNode small, ListNode large) = (new(0), new(0));
    var (cs, cl) = (small, large);
    while (head is not null)
    {
        if (head.val < x)
        {
            cs.next = head;
            cs = cs.next;
            head = head.next;
            cs.next = null;
        }
        else
        {
            cl.next = head;
            cl = cl.next;
            head = head.next;
            cl.next = null;
        }
    }
    cs.next = large.next;

    return small.next;
}

class ListNode : IEnumerable<int>
{
    public int val;
    public ListNode next;
    public ListNode(int val = 0, ListNode next = null)
    {
        this.val = val;
        this.next = next;
    }

    public static ListNode Make(IEnumerable<int> nums)
    {
        ListNode dummy = new(0, null);
        var curr = dummy;
        foreach (var num in nums)
        {
            var temp = new ListNode(num, null);
            curr.next = temp;
            curr = curr.next;
        }
        return dummy.next;
    }

    public IEnumerator<int> GetEnumerator()
    {
        var curr = this;
        while (curr is not null)
        {
            yield return curr.val;
            curr = curr.next;
        }
    }

    public override string ToString()
    {
        var curr = this;
        var sb = new StringBuilder("[");
        while (curr is ListNode n)
        {
            sb.AppendFormat("{0},", n.val);
            curr = n.next;
        }
        sb.Remove(sb.Length - 1, 1);
        sb.Append(']');
        return sb.ToString();
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }
}
