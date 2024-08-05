using System.Collections;
using System.Diagnostics;
using System.Text;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    ListNode n = DeleteDuplicates(ListNode.Make([1, 1, 2]));
    var a = "[1,2]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}

void Test2()
{
    ListNode n = DeleteDuplicates(ListNode.Make([1, 1, 2, 3, 3]));
    var a = "[1,2,3]";
    Debug.Assert(n.ToString() == a, $"Output: {n}\nExpect: {a}");
}


ListNode DeleteDuplicates(ListNode head)
{
        if (head is null) { return head; }

        var curr = head;
        while (curr is not null && curr.next is not null)
        {
            if (curr.val == curr.next.val)
            {
                curr.next = curr.next.next;
            }
            else
            {
                curr = curr.next;
            }
        }
        return head;
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
