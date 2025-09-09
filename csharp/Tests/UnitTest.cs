using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(5, 6, 2, 4)]
    [DataRow(6, 4, 1, 3)]
    public void TestMethod1(int exp, int n, int d, int f)
    {
        Assert.AreEqual(exp, sol.PeopleAwareOfSecret(n, d, f));
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