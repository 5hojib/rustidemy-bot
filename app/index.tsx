import React, { useState, useEffect, useMemo } from 'react';
import { createRoot } from 'react-dom/client';
import {
  ExternalLink,
  Tag,
  ChevronRight,
  X,
  RefreshCw,
  Zap,
  Copy,
  CheckCircle2,
  Sun,
  Moon,
  Loader2,
  WifiOff,
  Sparkles,
  Plus
} from 'lucide-react';

// --- Constants ---
const PAGE_SIZE = 10;
const MAX_COURSES = 50;

// --- Types ---
interface Course {
  id: string;
  title: string;
  description: string;
  thumbnail: string;
  link: string;
  category?: string;
  highResCover?: string;
  resolvedUdemyUrl?: string;
  createdAt: string;
}

// --- Components ---

const SkeletonCard = () => (
  <div className="bg-white dark:bg-gray-800 rounded-2xl border border-gray-100 dark:border-gray-700 overflow-hidden shadow-sm">
    <div className="aspect-video shimmer w-full" />
    <div className="p-5 space-y-3">
      <div className="h-5 shimmer w-3/4 rounded-md" />
      <div className="h-4 shimmer w-full rounded-md" />
      <div className="flex justify-between pt-4">
        <div className="h-3 shimmer w-1/4 rounded-md" />
        <div className="h-3 shimmer w-1/4 rounded-md" />
      </div>
    </div>
  </div>
);

interface CourseCardProps {
  course: Course;
  onClick: () => void;
}

const CourseCard: React.FC<CourseCardProps> = ({ course, onClick }) => {
  const isResolving = !course.highResCover;
  const displayImage = course.highResCover || course.thumbnail;

  return (
    <div
      onClick={onClick}
      className="group bg-white dark:bg-gray-800 rounded-2xl border border-gray-100 dark:border-gray-700 overflow-hidden shadow-sm hover:shadow-xl hover:-translate-y-1 transition-all duration-300 cursor-pointer flex flex-col h-full grid-item-enter"
    >
      <div className="relative aspect-video overflow-hidden bg-gray-100 dark:bg-gray-900">
        <img
          src={displayImage || 'https://images.unsplash.com/photo-1516321318423-f06f85e504b3?w=800&auto=format&fit=crop&q=60'}
          alt={course.title}
          loading="lazy"
          className={`w-full h-full object-cover transition-all duration-700 group-hover:scale-105 ${isResolving && !course.thumbnail ? 'opacity-0' : 'opacity-100'}`}
        />
        {isResolving && (
          <div className="absolute inset-0 flex items-center justify-center bg-black/5 backdrop-blur-[1px]">
             <Loader2 className="w-5 h-5 text-indigo-500 animate-spin" />
          </div>
        )}
        {course.highResCover && (
          <div className="absolute bottom-2 right-2 z-10 p-1 bg-emerald-500/80 backdrop-blur-sm rounded-lg text-white shadow-lg">
            <Sparkles size={10} />
          </div>
        )}
        <div className="absolute top-2 left-2 bg-indigo-600 text-white text-[9px] font-bold px-2 py-0.5 rounded uppercase tracking-wider shadow-lg">
          FREE
        </div>
      </div>
      <div className="p-4 flex flex-col flex-grow">
        <h3 className="font-bold text-gray-900 dark:text-gray-100 text-sm leading-snug mb-2 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors line-clamp-2">
          {course.title}
        </h3>
        <div className="flex items-center justify-between mt-auto pt-3 border-t border-gray-50 dark:border-gray-700">
          <span className="text-[10px] font-bold text-gray-400 dark:text-gray-500 flex items-center gap-1 uppercase tracking-tight">
            <Tag size={10} className="text-indigo-400" /> {course.category || 'Course'}
          </span>
          <span className="text-indigo-600 dark:text-indigo-400 text-xs font-bold flex items-center gap-0.5 group-hover:translate-x-0.5 transition-transform">
            View <ChevronRight size={14} />
          </span>
        </div>
      </div>
    </div>
  );
};

const CourseDetail = ({ course, onClose }: { course: Course; onClose: () => void }) => {
  const [copied, setCopied] = useState(false);
  if (!course) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      <div className="absolute inset-0 bg-gray-950/80 backdrop-blur-md animate-in fade-in" onClick={onClose} />
      <div className="relative bg-white dark:bg-gray-900 w-full max-w-xl max-h-[90vh] rounded-3xl shadow-2xl overflow-hidden flex flex-col modal-enter border dark:border-gray-800">
        <button onClick={onClose} className="absolute top-4 right-4 z-20 p-2 bg-white/20 hover:bg-white/40 backdrop-blur-xl rounded-full text-white transition-all"><X size={20} /></button>
        <div className="overflow-y-auto flex-grow">
          <div className="relative aspect-video w-full overflow-hidden">
            <img src={course.highResCover || course.thumbnail} alt={course.title} className="w-full h-full object-cover" />
            <div className="absolute inset-0 bg-gradient-to-t from-gray-950 via-gray-950/20 to-transparent flex items-end p-6">
              <h2 className="text-xl sm:text-2xl font-bold text-white leading-tight">{course.title}</h2>
            </div>
          </div>
          <div className="p-6 space-y-6">
            <div className="flex flex-wrap gap-2">
              <div className="flex items-center gap-1.5 px-2.5 py-1 bg-indigo-50 dark:bg-indigo-900/30 text-indigo-700 dark:text-indigo-300 rounded-lg text-[10px] font-bold uppercase tracking-wide"><Zap size={12} fill="currentColor" /> 100% OFF</div>
              <div className="flex items-center gap-1.5 px-2.5 py-1 bg-emerald-50 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300 rounded-lg text-[10px] font-bold uppercase tracking-wide"><CheckCircle2 size={12} /> Verified</div>
            </div>
            <p className="text-gray-600 dark:text-gray-300 text-base leading-relaxed">{course.description}</p>
            <div className="flex flex-col sm:flex-row items-center gap-3 pt-2">
              <a href={course.resolvedUdemyUrl || course.link} target="_blank" rel="noopener noreferrer" className="w-full sm:flex-1 text-white text-center py-3.5 rounded-2xl font-bold bg-indigo-600 hover:bg-indigo-700 transition-all shadow-lg flex items-center justify-center gap-2">Enroll Now <ExternalLink size={18} /></a>
              <button onClick={() => { navigator.clipboard.writeText(course.resolvedUdemyUrl || course.link); setCopied(true); setTimeout(() => setCopied(false), 2000); }} className="w-full sm:w-auto px-6 py-3.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-200 rounded-2xl font-bold flex items-center justify-center gap-2">
                {copied ? <CheckCircle2 size={18} className="text-emerald-500" /> : <Copy size={18} />} {copied ? 'Copied' : 'Link'}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

function App() {
  const [courses, setCourses] = useState<Course[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [visibleCount, setVisibleCount] = useState(PAGE_SIZE);
  const [selectedCourseId, setSelectedCourseId] = useState<string | null>(null);
  const [theme, setTheme] = useState<'light' | 'dark'>(() => localStorage.getItem('theme') as 'light' | 'dark' || 'light');

  useEffect(() => {
    theme === 'dark' ? document.documentElement.classList.add('dark') : document.documentElement.classList.remove('dark');
    localStorage.setItem('theme', theme);
  }, [theme]);

  const fetchData = async () => {
    setLoading(true);
    setError(null);
    setVisibleCount(PAGE_SIZE);
    try {
      const response = await fetch('/api/courses');
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      const data = await response.json();
      setCourses(data);
    } catch (err: any) {
      setError(err.message || "Failed to fetch deals.");
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { fetchData(); }, []);

  const visibleCourses = useMemo(() => courses.slice(0, visibleCount), [courses, visibleCount]);
  const selectedCourse = useMemo(() => courses.find(c => c.id === selectedCourseId), [courses, selectedCourseId]);

  return (
    <div className="min-h-screen pb-20 bg-gray-50 dark:bg-gray-950 transition-colors duration-300">
      <header className="sticky top-0 z-40 bg-white/80 dark:bg-gray-900/80 backdrop-blur-xl border-b border-gray-100 dark:border-gray-800">
        <div className="max-w-6xl mx-auto px-4 h-16 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-8 h-8 bg-indigo-600 rounded-lg flex items-center justify-center text-white"><Zap size={18} fill="currentColor" /></div>
            <h1 className="text-lg font-black tracking-tight dark:text-white">Udemy<span className="text-indigo-600">Hunter</span></h1>
          </div>
          <div className="flex items-center gap-1">
            <button onClick={() => setTheme(t => t === 'light' ? 'dark' : 'light')} className="p-2 text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-all">{theme === 'light' ? <Moon size={20} /> : <Sun size={20} />}</button>
            <button onClick={fetchData} disabled={loading} className="p-2 text-gray-500 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-xl transition-all"><RefreshCw size={20} className={loading ? 'animate-spin' : ''} /></button>
          </div>
        </div>
      </header>

      <main className="max-w-6xl mx-auto px-4 pt-10">
        <div className="mb-10 text-center md:text-left">
          <h2 className="text-3xl font-black text-gray-900 dark:text-white mb-2">Latest Free Deals</h2>
          <p className="text-gray-500 dark:text-gray-400 font-medium">Auto-collecting 100% off coupons. Top {MAX_COURSES} unique deals.</p>
        </div>

        {error ? (
          <div className="py-20 text-center bg-white dark:bg-gray-900 rounded-3xl border border-dashed border-gray-200 dark:border-gray-800">
            <WifiOff size={40} className="mx-auto text-red-400 mb-4" />
            <h3 className="text-xl font-bold dark:text-white">Connection Error</h3>
            <button onClick={fetchData} className="mt-4 px-6 py-2 bg-indigo-600 text-white rounded-xl font-bold">Retry</button>
          </div>
        ) : (
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 xl:grid-cols-5 gap-6">
            {loading ? Array.from({ length: 10 }).map((_, i) => <SkeletonCard key={i} />) :
              visibleCourses.map((course) => <CourseCard key={course.id} course={course} onClick={() => setSelectedCourseId(course.id)} />)}
          </div>
        )}

        {!loading && !error && visibleCount < courses.length && (
          <div className="mt-12 flex justify-center">
            <button onClick={() => setVisibleCount(c => c + PAGE_SIZE)} className="px-8 py-4 bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 text-gray-900 dark:text-white rounded-2xl font-bold hover:shadow-lg transition-all flex items-center gap-2">
              <Plus size={20} /> Load More Deals
            </button>
          </div>
        )}
      </main>

      {selectedCourse && <CourseDetail course={selectedCourse} onClose={() => setSelectedCourseId(null)} />}
    </div>
  );
}

const container = document.getElementById('root');
const root = createRoot(container!);
root.render(<App />);
