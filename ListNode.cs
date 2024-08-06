using System.Collections;
using System.Text;

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
