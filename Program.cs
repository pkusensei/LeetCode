using System.Diagnostics;

Test1();
Test2();
Test3();

Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var n = Node.Make([[7, null], [13, 0], [11, 4], [10, 2], [1, 0]]);
    var s = CopyRandomList(n);
    var a = "[[7,null],[13,0],[11,4],[10,2],[1,0]]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test2()
{
    var n = Node.Make([[1, 1], [2, 1]]);
    var s = CopyRandomList(n);
    var a = "[[1,1],[2,1]]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}

void Test3()
{
    var n = Node.Make([[3, null], [3, 0], [3, null]]);
    var s = CopyRandomList(n);
    var a = "[[3,null],[3,0],[3,null]]";
    Debug.Assert(s.ToString() == a, $"Output: {s}\nExpect: {a}");
}


Node CopyRandomList(Node head)
{
    if (head is null) { return null; }

    var curr = head;
    while (curr is not null)
    {
        Node temp = new(curr.val)
        {
            random = curr.random,
            next = curr.next
        };
        curr.next = temp;
        curr = temp.next;
    }

    curr = head.next;
    while (curr.next is not null)
    {
        if (curr.random is not null)
        {
            curr.random = curr.random.next;
        }
        curr = curr.next.next;
    }
    if (curr.random is not null)
    {
        curr.random = curr.random.next;
    }

    var old = head;
    var res = head.next;
    curr = res;
    while (curr.next is not null)
    {
        old.next = curr.next;
        old = old.next;
        curr.next = curr.next.next;
        curr = curr.next;
    }
    old.next = null;

    return res;
}