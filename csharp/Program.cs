using System.Diagnostics;
using LList;
using Tree;

Test1();
Test2();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var iter = new int[] { 1, 2, 3 }.AsEnumerable().GetEnumerator();
    iter.MoveNext(); // WTF
    PeekingIterator it = new(iter); // [1,2,3]
    Debug.Assert(it.Next == 1);    // return 1, the pointer moves to the next element [1,2,3].
    Debug.Assert(it.Peek == 2);    // return 2, the pointer does not move [1,2,3].
    Debug.Assert(it.Next == 2);    // return 2, the pointer moves to the next element [1,2,3]
    Debug.Assert(it.Next == 3);    // return 3, the pointer moves to the next element [1,2,3]
    Debug.Assert(!it.HasNext); // return False
}

void Test2()
{
}

class PeekingIterator
{
    private IEnumerator<int> Iterator { get; }
    private int? _peek;

    public PeekingIterator(IEnumerator<int> iterator)
    {
        Iterator = iterator;
        _peek = Iterator.Current;
    }

    public int Peek
    {
        get
        {
            if (_peek is int v) { return v; }
            Iterator.MoveNext();
            _peek = Iterator.Current;
            return _peek.Value;
        }
    }

    public int Next
    {
        get
        {
            if (_peek is int v)
            {
                _peek = null;
                return v;
            }
            Iterator.MoveNext();
            return Iterator.Current;
        }
    }

    public bool HasNext
    {
        get
        {
            if (_peek.HasValue) { return true; }
            var hasNext = Iterator.MoveNext();
            if (hasNext)
            {
                _peek = Iterator.Current;
                return true;
            }
            else { return false; }
        }
    }
}