using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(2, 5, 25)]
    [DataRow(3, 7, 499)]
    [DataRow(7, 17, 20379000)]
    public void TestMethod1(int k, int n, int exp)
    {
        Assert.AreEqual(exp, sol.KMirror(k, n));
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