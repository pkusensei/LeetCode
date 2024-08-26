using System.Collections;
using System.Text;

public class ListNodeBase<T> where T : ListNodeBase<T>
{
    public int val;
    public T next;
}

public class ListNode : ListNodeBase<ListNode>, IEnumerable<int>
{
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

    public static ListNode Make(IEnumerable<int> nums, int pos)
    {
        ListNode dummy = new(0, null);
        var curr = dummy;
        ListNode loop = null;
        var count = 0;
        foreach (var num in nums)
        {
            var temp = new ListNode(num, null);
            curr.next = temp;
            curr = curr.next;
            if (pos == count) { loop = curr; }
            count += 1;
        }
        curr.next = loop;
        return dummy.next;
    }

    public ListNode FindMiddleNode()
    {
        var curr = this;
        if (curr is null || curr.next is null) { return curr; }

        (var slow, var fast) = (curr, curr);
        while (fast.next is not null && fast.next.next is not null)
        {
            slow = slow.next;
            fast = fast.next.next;
        }
        return slow;
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

public class Node : ListNodeBase<Node>
{
    public Node random;

    public Node(int _val = 0) { val = _val; next = null; random = null; }

    public static Node Make(IList<IList<int?>> values)
    {
        var nodes = new Dictionary<int, (int?, Node)>();
        Node dummy = new(0);
        var curr = dummy;
        foreach (var (idx, pair) in values.Select((v, i) => (i, v)))
        {
            var node = new Node(pair[0].Value);
            curr.next = node;
            curr = curr.next;
            nodes.Add(idx, (pair[1], node));
        }
        foreach (var (num, node) in nodes.Values)
        {
            if (num.HasValue)
            {
                node.random = nodes[num.Value].Item2;
            }
        }
        return nodes[0].Item2;
    }

    public override string ToString()
    {
        var nodes = new Dictionary<Node, int>();
        var curr = this;
        var count = 0;
        while (curr is Node n)
        {
            nodes.Add(curr, count);
            curr = curr.next;
            count += 1;
        }

        curr = this;
        var sb = new StringBuilder("[");
        while (curr is Node n)
        {
            sb.AppendFormat("[{0},", n.val);
            if (n.random is null)
            {
                sb.Append("null],");
            }
            else
            {
                sb.AppendFormat("{0}],", nodes[curr.random]);
            }
            curr = n.next;
        }
        sb.Replace(',', ']', sb.Length - 1, 1);
        return sb.ToString();
    }
}