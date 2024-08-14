using System.Diagnostics;
using System.Text;

Test1();
Test2();
Test3();
Test4();
Console.WriteLine("!!Test Passed!!");

void Test1()
{
    var cache = new LRUCache(2);
    cache.Put(1, 1); // cache is {1=1}
    cache.Put(2, 2); // cache is {1=1, 2=2}
    Debug.Assert(cache.Get(1) == 1);    // return 1
    cache.Put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
    Debug.Assert(cache.Get(2) == -1);    // returns -1 (not found)
    cache.Put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
    Debug.Assert(cache.Get(1) == -1);    // return -1 (not found)
    Debug.Assert(cache.Get(3) == 3);    // return 3
    Debug.Assert(cache.Get(4) == 4);    // return 4
}

void Test2()
{
}

void Test3()
{
}

void Test4()
{
}

public class LRUCache
{
    public LinkedList<(int k, int v)> List { get; }
    public Dictionary<int, LinkedListNode<(int k, int v)>> Map { get; }
    public int Capacity { get; }
    public int Count { get; private set; }

    public LRUCache(int capacity)
    {
        List = [];
        Map = [];
        Capacity = capacity;
        Count = 0;
    }

    public int Get(int key)
    {
        if (Map.TryGetValue(key, out var node))
        {
            List.Remove(node);
            List.AddFirst(node);
            return node.Value.v;
        }
        return -1;
    }

    public void Put(int key, int value)
    {
        if (Map.TryGetValue(key, out var node))
        {
            List.Remove(node);
            List.AddFirst((key, value));
            Map[key] = List.First;
        }
        else
        {
            Count += 1;
            if (Count > Capacity)
            {
                var lru = List.Last;
                List.RemoveLast();
                Map.Remove(lru.Value.k);
            }
            List.AddFirst((key, value));
            Map.Add(key, List.First);
        }
    }
}
