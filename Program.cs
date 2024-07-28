// See https://aka.ms/new-console-template for more information

using System.Text;

Test1();
Test2();
Test3();


void Test1()
{
    ListNode n1 = new(1, new(2, new(4)));
    ListNode n2 = new(1, new(3, new(4)));
    var ans = MergeTwoLists(n1, n2);
    Console.WriteLine(ans); // [1,1,2,3,4,4]
}

void Test2()
{
    ListNode n1 = null;
    ListNode n2 = null;
    var ans = MergeTwoLists(n1, n2);
    Console.WriteLine(ans);
}

void Test3()
{
    ListNode n1 = null;
    ListNode n2 = new(0);
    var ans = MergeTwoLists(n1, n2);
    Console.WriteLine(ans); // [0]
}


ListNode MergeTwoLists(ListNode list1, ListNode list2)
{
    if (list1 is null) { return list2; }
    if (list2 is null) { return list1; }

    ListNode head;
    if (list1.val < list2.val)
    {
        head = list1;
        list1 = list1.next;
        head.next = MergeTwoLists(list1, list2);
    }
    else
    {
        head = list2;
        list2 = list2.next;
        head.next = MergeTwoLists(list1, list2);
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
