using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(1, 6000, 4, "124", 5)]
    [DataRow(15, 215, 6, "10", 2)]
    [DataRow(1000, 2000, 4, "3000", 0)]
    public void TestMethod1(long start, long finish, int limit, string s, long exp)
    {
        Assert.AreEqual(exp, sol.NumberOfPowerfulInt(start, finish, limit, s));
    }

    [TestMethod]
    public void TestMethod2()
    { }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}