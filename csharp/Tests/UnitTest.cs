using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(8, new[] { 'a', 'a', 'a', 'b', 'b', 'b' }, 2)]
    public void TestMethod1(int exp, char[] t, int n)
    {
        Assert.AreEqual(exp, sol.LeastInterval(t, n));
    }

    [TestMethod]
    public void TestMethod2()
    {
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}