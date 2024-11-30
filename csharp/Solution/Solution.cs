using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class H2O
{
    int count = 0;
    readonly SemaphoreSlim hy = new(1, 1);
    readonly SemaphoreSlim ox = new(0, 1);

    public H2O() { }

    public void Hydrogen(Action releaseHydrogen)
    {
        hy.Wait();
        releaseHydrogen();
        count += 1;
        if (count == 1) { hy.Release(); }
        else { ox.Release(); }
    }

    public void Oxygen(Action releaseOxygen)
    {
        ox.Wait();
        // releaseOxygen() outputs "O". Do not change or remove this line.
        releaseOxygen();
        count = 0;
        hy.Release();
    }
}
