using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

class PeekingIterator
{
    IEnumerator<int> It { get; set; }

    // iterators refers to the first element of the array.
    public PeekingIterator(IEnumerator<int> iterator)
    {
        // initialize any member here.
        It = iterator;
    }

    // Returns the next element in the iteration without advancing the iterator.
    public int Peek() => It?.Current ?? -1;

    // Returns the next element in the iteration and advances the iterator.
    public int Next()
    {
        int val = It.Current;
        if (!It.MoveNext()) { It = null; }
        return val;
    }

    // Returns false if the iterator is refering to the end of the array of true otherwise.
    public bool HasNext() => It is not null;
}